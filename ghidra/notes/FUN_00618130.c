
void * __thiscall FUN_00618130(void *this,int param_1,int param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00657302;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e620;
    *(undefined **)((int)this + 0xc) = &DAT_0066e618;
    FUN_006153b0((undefined4 *)((int)this + 0x14));
    local_4 = 0;
  }
  FUN_006179e0(this,param_1,0);
  local_4 = 1;
  FUN_00616210((void *)((int)this + 0xc),param_1,0);
  *(undefined ***)(*(int *)(*(int *)this + 4) + (int)this) = &PTR_LAB_0066e614;
  ExceptionList = local_c;
  return this;
}

