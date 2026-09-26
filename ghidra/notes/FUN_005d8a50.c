
void __fastcall FUN_005d8a50(undefined4 *param_1)

{
  uint uVar1;
  int iVar2;
  void *unaff_EBX;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00654aaf;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066d320;
  param_1[7] = &PTR_LAB_0066d310;
  local_4 = 1;
  uVar1 = (**(code **)(*(int *)param_1[0xb] + 8))((int *)param_1[0xb]);
  iVar2 = (**(code **)(*(int *)param_1[0xc] + 0x98))((int *)param_1[0xc],param_1[0xb],0);
  FUN_005d8b00(iVar2);
  if (1 < uVar1) {
    iVar2 = (**(code **)(*(int *)param_1[0xb] + 8))((int *)param_1[0xb]);
    while (iVar2 != 0) {
      iVar2 = (**(code **)(*(int *)param_1[0xb] + 8))((int *)param_1[0xb]);
    }
  }
  FUN_005f52b0((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 7)));
  FUN_005f55f0(param_1);
  ExceptionList = unaff_EBX;
  return;
}

