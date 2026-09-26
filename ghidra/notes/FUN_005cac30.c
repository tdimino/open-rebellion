
void __cdecl FUN_005cac30(int param_1)

{
  uint uVar1;
  undefined4 *puVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654266;
  local_c = ExceptionList;
  uVar1 = **(uint **)(param_1 + 4);
  ExceptionList = &local_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar2 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar2);
    }
  }
  local_4 = 0xffffffff;
  iVar3 = FUN_005a6480(DAT_006bc470,uVar1);
  if (iVar3 != 0) {
    puVar2 = (undefined4 *)FUN_00618b70(0xc);
    local_4 = 1;
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      FUN_005c6150(puVar2);
      *puVar2 = &PTR_FUN_0066cda0;
    }
    local_4 = 0xffffffff;
    if (puVar2 != (undefined4 *)0x0) {
      FUN_005d1e40((void *)(iVar3 + 0x8c),(int)puVar2);
    }
  }
  ExceptionList = local_c;
  return;
}

