
undefined4 __thiscall FUN_0055a960(void *this,int param_1,int *param_2,int *param_3,void *param_4)

{
  int *piVar1;
  void *pvVar2;
  int iVar3;
  undefined1 local_24 [8];
  undefined1 local_1c [8];
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649aa8;
  local_c = ExceptionList;
  iVar3 = 0;
  ExceptionList = &local_c;
  if (*param_2 != 0) {
    ExceptionList = &local_c;
    piVar1 = FUN_0052e740(this,local_24);
    iVar3 = ((param_2[1] + -1) * *piVar1) / *param_2;
    FUN_00619730();
  }
  pvVar2 = FUN_0052e740(this,local_24);
  *param_3 = *(int *)((int)pvVar2 + 4) - iVar3;
  FUN_00619730();
  if (*param_3 < 0) {
    *param_3 = 0;
  }
  if (param_1 < *param_3) {
    *param_3 = param_1;
  }
  if (0 < *param_3) {
    piVar1 = (int *)FUN_0051c900(local_14,0,-*param_3);
    local_4 = 0;
    pvVar2 = FUN_0052e740(this,local_1c);
    local_4._0_1_ = 1;
    piVar1 = FUN_0051ca00(pvVar2,local_24,piVar1);
    local_4._0_1_ = 2;
    FUN_0055a6e0(this,piVar1,param_4);
    local_4._0_1_ = 1;
    FUN_00619730();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return 1;
}

