
int __thiscall FUN_00552f40(void *this,uint *param_1,void *param_2,int *param_3)

{
  int iVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648f28;
  local_c = ExceptionList;
  iVar1 = 1;
  ExceptionList = &local_c;
  FUN_00520580(param_2,(undefined4 *)((int)this + 0xc));
  *param_3 = 0;
  if (*(int *)((int)param_2 + 4) == -1) {
    iVar1 = FUN_00553130(this,param_1,param_3);
  }
  if (((iVar1 != 0) && (*(int *)((int)param_2 + 4) == -1)) && (*param_3 == 0)) {
    local_14 = 1;
    local_10 = 0x12;
    local_4 = 0;
    FUN_00520580(param_2,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar1;
}

