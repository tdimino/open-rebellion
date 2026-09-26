
int __thiscall FUN_00586720(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint uVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f718;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  bVar2 = FUN_00521030(this,(int *)&param_1);
  iVar4 = 0;
  if (CONCAT31(extraout_var,bVar2) != 0) {
    local_4 = 0;
    uVar3 = (**(code **)(*param_1 + 4))();
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      iVar4 = 0;
    }
    else {
      iVar4 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar4 != 0) {
      *piVar1 = (int)param_1;
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

