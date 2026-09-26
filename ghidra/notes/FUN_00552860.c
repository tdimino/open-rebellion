
int __thiscall
FUN_00552860(void *this,int param_1,int param_2,uint *param_3,void *param_4,void *param_5)

{
  void *this_00;
  void *this_01;
  int iVar1;
  undefined4 *puVar2;
  int local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_4;
  puStack_8 = &LAB_00648e28;
  local_c = ExceptionList;
  local_18[1] = 1;
  local_18[2] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,local_18 + 1);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_01 = param_5;
  FUN_0054fb20((int)param_5);
  local_18[0] = 0;
  iVar1 = FUN_005501e0(param_1);
  param_4 = (void *)0x0;
  if (iVar1 != 0) {
    if (*(int *)((int)this_00 + 4) == -1) {
      iVar1 = FUN_00552e80(this,param_3,this_00,(int *)&param_4);
    }
    if (iVar1 != 0) {
      if (*(int *)((int)this_00 + 4) == -1) {
        iVar1 = FUN_00552b10(this,param_4,param_1,param_2,local_18,this_01);
      }
      if ((iVar1 != 0) && (*(int *)((int)this_00 + 4) == -1)) {
        puVar2 = FUN_0054fc40(this_01,local_18 + 1);
        local_4 = 1;
        FUN_00520580(this_00,puVar2);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  if (iVar1 == 0) {
    local_18[1] = 1;
    local_18[2] = 1;
    local_4 = 2;
    FUN_00520580(this_00,local_18 + 1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar1;
}

