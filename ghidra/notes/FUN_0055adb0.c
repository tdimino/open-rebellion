
bool __fastcall FUN_0055adb0(int *param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  int iVar4;
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649b28;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0053aca0(param_1);
  iVar1 = param_1[0x16];
  if ((((*(byte *)(param_1 + 0x14) & 1) != 0) && ((*(byte *)(param_1 + 0x18) & 1) == 0)) &&
     (((byte)param_1[9] & 0xc0) != 0xc0)) {
    pvVar3 = FUN_0052e740(param_1,local_14);
    local_4 = 0;
    iVar4 = (**(code **)(*param_1 + 0x214))(pvVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar4 != 0) {
      bVar2 = true;
      goto LAB_0055ae38;
    }
  }
  bVar2 = false;
LAB_0055ae38:
  ExceptionList = local_c;
  return (iVar1 != 4) == bVar2;
}

