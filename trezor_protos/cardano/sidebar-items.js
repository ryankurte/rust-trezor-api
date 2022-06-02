initSidebarItems({"enum":[["CardanoAddressType","Values correspond to address header values given by the spec. Script addresses are only supported in transaction outputs."],["CardanoCertificateType",""],["CardanoDerivationType",""],["CardanoNativeScriptHashDisplayFormat",""],["CardanoNativeScriptType",""],["CardanoPoolRelayType",""],["CardanoTxAuxiliaryDataSupplementType",""],["CardanoTxSigningMode",""],["CardanoTxWitnessType",""]],"struct":[["CardanoAddress","Request: Ask device for Cardano address @end"],["CardanoAddressParametersType","Structure to represent address parameters so they can be reused in CardanoGetAddress and CardanoTxOutputType. NetworkId isn’t a part of the parameters, because in a transaction this will be included separately in the transaction itself, so it shouldn’t be duplicated here. @embed"],["CardanoAssetGroup","Request: Transaction output asset group data @next CardanoTxItemAck"],["CardanoBlockchainPointerType","Structure representing cardano PointerAddress pointer, which points to a staking key registration certificate. @embed"],["CardanoCatalystRegistrationParametersType","@embed"],["CardanoGetAddress","Request: Ask device for Cardano address @start @next CardanoAddress @next Failure"],["CardanoGetNativeScriptHash","Request: Ask device for Cardano native script hash @start @next CardanoNativeScriptHash @next Failure"],["CardanoGetPublicKey","Request: Ask device for public key corresponding to address_n path @start @next CardanoPublicKey @next Failure"],["CardanoNativeScript","@embed"],["CardanoNativeScriptHash","Request: Ask device for Cardano native script hash @end"],["CardanoPoolMetadataType","Stake pool metadata parameters @embed"],["CardanoPoolOwner","Request: Stake pool owner parameters @next CardanoTxItemAck"],["CardanoPoolParametersType","Stake pool parameters @embed"],["CardanoPoolRelayParameters","Request: Stake pool relay parameters @next CardanoTxItemAck"],["CardanoPublicKey","Response: Contains public key derived from device private seed @end"],["CardanoSignTxFinished","Response: Confirm the successful completion of the signing process @end"],["CardanoSignTxInit","Request: Initiate the Cardano transaction signing process on the device @start @next CardanoTxItemAck @next Failure"],["CardanoToken","Request: Transaction output asset group token data @next CardanoTxItemAck"],["CardanoTxAuxiliaryData","Request: Transaction auxiliary data @next CardanoTxItemAck @next CardanoTxAuxiliaryDataSupplement"],["CardanoTxAuxiliaryDataSupplement","Response: Device-generated supplement for the auxiliary data @next CardanoTxWitnessRequest"],["CardanoTxBodyHash","Response: Hash of the serialized transaction body @next CardanoTxHostAck"],["CardanoTxCertificate","Request: Transaction certificate data @next CardanoTxItemAck"],["CardanoTxCollateralInput","Request: Transaction collateral input data @next CardanoTxItemAck"],["CardanoTxHostAck","Request: Acknowledgement of the last response received @next CardanoTxBodyHash @next CardanoSignTxFinished"],["CardanoTxInput","Request: Transaction input data @next CardanoTxItemAck"],["CardanoTxItemAck","Response: Acknowledgement of the last transaction item received @next CardanoTxInput @next CardanoTxOutput @next CardanoAssetGroup @next CardanoToken @next CardanoTxCertificate @next CardanoPoolOwner @next CardanoPoolRelayParameters @next CardanoTxWithdrawal @next CardanoTxAuxiliaryData @next CardanoTxWitnessRequest @next CardanoTxMint @next CardanoTxCollateralInput @next CardanoTxRequiredSigner"],["CardanoTxMint","Request: Transaction mint @next CardanoTxItemAck"],["CardanoTxOutput","Request: Transaction output data @next CardanoTxItemAck"],["CardanoTxRequiredSigner","Request: Transaction required signer @next CardanoTxItemAck"],["CardanoTxWithdrawal","Request: Transaction withdrawal data @next CardanoTxItemAck"],["CardanoTxWitnessRequest","Request: Ask the device to sign a witness path @next CardanoTxWitnessResponse"],["CardanoTxWitnessResponse","Response: Signature corresponding to the requested witness path @next CardanoTxWitnessRequest @next CardanoTxHostAck"]]});