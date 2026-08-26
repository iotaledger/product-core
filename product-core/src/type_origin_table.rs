use std::{borrow::Cow, collections::HashMap};

use iota_sdk::{
    graphql_client::{Client as IotaClient, error::Error as IotaClientError},
    types::{ObjectId, StructTag, TypeOrigin, TypeTag},
};

pub type TypeData = TypeDataRef<'static>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TypeDataRef<'a> {
    module: Cow<'a, str>,
    name: Cow<'a, str>,
}

impl<'a> TypeDataRef<'a> {
    fn from_struct_tag(struct_tag: &'a StructTag) -> Self {
        Self {
            module: struct_tag.module().as_str().into(),
            name: struct_tag.name().as_str().into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeOriginTable(HashMap<TypeData, ObjectId>);

impl TypeOriginTable {
    pub async fn new(
        latest_package_id: ObjectId,
        client: &IotaClient,
    ) -> Result<Self, TypeOriginTableCreationError> {
        let package = client
            .package(latest_package_id.into(), None)
            .await?
            .ok_or(TypeOriginTableCreationError::PackageNotFound(
                latest_package_id,
            ))?;

        Ok(Self(
            package
                .type_origin_table
                .into_iter()
                .map(type_origin_mapping)
                .collect(),
        ))
    }

    pub fn canonicalize_type(&self, type_tag: &mut TypeTag) {
        match type_tag {
            TypeTag::Vector(arg_type) => self.canonicalize_type(arg_type.as_mut()),
            TypeTag::Struct(struct_tag) => {
                let canonical_pkg_id = self
                    .0
                    .get(&TypeDataRef::from_struct_tag(&*struct_tag))
                    .copied()
                    .unwrap_or_else(|| struct_tag.address().into());

                let (_, module, name, mut type_args) =
                    std::mem::replace(struct_tag.as_mut(), StructTag::new_id()).into_parts();
                type_args
                    .iter_mut()
                    .for_each(|arg_type| self.canonicalize_type(arg_type));
                **struct_tag = StructTag::new(canonical_pkg_id, module, name, type_args);
            }
            _ => (),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TypeOriginTableCreationError {
    #[error("package '{0}' does not exists")]
    PackageNotFound(ObjectId),
    #[error(transparent)]
    ClientError(#[from] IotaClientError),
}

fn type_origin_mapping(type_origin: TypeOrigin) -> (TypeData, ObjectId) {
    let TypeOrigin {
        module_name,
        datatype_name,
        package,
    } = type_origin;
    let module = module_name.to_string().into();
    let name = datatype_name.to_string().into();
    (TypeData { module, name }, package)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn constructing_type_origin_table_for_identity_testnet_works() -> anyhow::Result<()> {
        let client = IotaClient::new_testnet();
        let identity_pkg_id =
            "0x29359d33a2e84f04407da0d6cff15dd8ad271c75493ef6b78f381993e4c0abb0".parse()?;
        let _ = TypeOriginTable::new(identity_pkg_id, &client).await?;

        Ok(())
    }
}
