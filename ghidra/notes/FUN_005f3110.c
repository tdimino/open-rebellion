
void * __thiscall FUN_005f3110(void *this,void *param_1,int param_2)

{
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065557f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(&local_18);
  local_4 = 1;
  local_10 = FUN_005f36d0(*(char **)((int)this + 4),*(char **)(param_2 + 4),&local_14);
  FUN_005f2f90(param_1,(int)&local_18);
  local_4 = local_4 & 0xffffff00;
  FUN_005f2ff0(&local_18);
  ExceptionList = local_c;
  return param_1;
}

