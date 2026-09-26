
bool __cdecl FUN_005400f0(int param_1,void *param_2,int param_3,int *param_4)

{
  int *piVar1;
  int iVar2;
  uint *puVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646a68;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  *param_4 = 0;
  if (DAT_006b90e0 != 0) {
    iVar2 = FUN_0051f730(param_1);
    bVar4 = iVar2 != 0;
    *piVar1 = iVar2;
    if (bVar4) {
      *(uint *)(iVar2 + 0x18) = *(uint *)((int)param_2 + 0x24) >> 4 & 3;
      puVar3 = FUN_004025b0(param_2,(uint *)&param_4);
      local_4 = 0;
      FUN_004f26d0((void *)(*piVar1 + 0x20),puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      *(undefined4 *)(*piVar1 + 0xc) = 0;
      *(undefined4 *)(*piVar1 + 0x10) = *(undefined4 *)(param_3 + 0x14);
      *(undefined4 *)(*piVar1 + 0x14) = *(undefined4 *)(param_3 + 0x18);
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

