
uint __fastcall FUN_004efd30(int *param_1)

{
  uint uVar1;
  int iVar2;
  uint *puVar3;
  int *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ef18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = param_1;
  uVar1 = FUN_00534fa0(param_1);
  if (uVar1 != 0) {
    iVar2 = FUN_004ece60((uint *)(param_1 + 0x27));
    uVar1 = (uint)(iVar2 == 0);
    if (uVar1 != 0) {
      puVar3 = (uint *)FUN_004ed270(param_1,&local_10);
      local_4 = 0;
      iVar2 = FUN_004ece60(puVar3);
      uVar1 = (uint)(iVar2 == 0);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar1;
}

