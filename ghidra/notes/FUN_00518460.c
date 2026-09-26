
undefined4 FUN_00518460(void)

{
  void *this;
  void *this_00;
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642a78;
  local_c = ExceptionList;
  uVar3 = 1;
  ExceptionList = &local_c;
  this = (void *)FUN_00506f30(1);
  this_00 = (void *)FUN_00506f30(2);
  iVar1 = thunk_FUN_00506e10();
  if (((this != (void *)0x0) && (this_00 != (void *)0x0)) && (iVar1 != 0)) {
    FUN_004fd450(&local_28,iVar1 + 0x194);
    uVar3 = 0;
    local_4 = 0;
    local_24 = 1;
    local_20 = 1;
    iVar2 = FUN_0052f600(this,*(int *)(iVar1 + 0x74),&local_28);
    iVar1 = FUN_0052f600(this_00,*(int *)(iVar1 + 0x74),&local_28);
    if ((iVar1 != 0) && (iVar2 != 0)) {
      uVar3 = 1;
    }
    local_4 = 0xffffffff;
    FUN_004fd4d0(&local_28);
  }
  ExceptionList = local_c;
  return uVar3;
}

