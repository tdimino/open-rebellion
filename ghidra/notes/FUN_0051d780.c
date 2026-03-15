
undefined4 __thiscall FUN_0051d780(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643168;
  local_c = ExceptionList;
  if (*(int *)((int)this + 0xc0) == 0) {
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_0051c740(local_20);
    local_4 = 0;
    FUN_0051c790(param_1,pvVar1);
    local_4 = 0xffffffff;
    FUN_0051c780(local_20);
  }
  else {
    ExceptionList = &local_c;
    FUN_0051c790(param_1,(void *)(*(int *)((int)this + 0xc0) + 0x58));
  }
  ExceptionList = local_c;
  return 1;
}

