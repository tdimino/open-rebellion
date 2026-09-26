
int FUN_0051acb0(void *param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  uint local_1c;
  int local_18;
  int local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642e78;
  local_c = ExceptionList;
  local_1c = 0x92000121;
  local_4 = 0;
  ExceptionList = &local_c;
  piVar1 = (int *)FUN_00505f60(&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_10 = 0;
  local_14 = 0;
  local_18 = 0;
  iVar2 = FUN_0055d520(&local_10,&local_14,&local_18);
  if ((iVar2 == 0) || (piVar1 == (int *)0x0)) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  if (piVar1 != (int *)0x0) {
    iVar3 = FUN_0051be90(piVar1,local_10,local_14,local_18,param_1);
    if ((iVar3 == 0) || (iVar2 == 0)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
  }
  ExceptionList = local_c;
  return iVar2;
}

