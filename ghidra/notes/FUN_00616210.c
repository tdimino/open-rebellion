
void * __thiscall FUN_00616210(void *this,int param_1,int param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006571e6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e550;
    FUN_006153b0((undefined4 *)((int)this + 8));
    local_4 = 0;
  }
                    /* WARNING: Load size is inaccurate */
  *(undefined ***)(*(int *)(*this + 4) + (int)this) = &PTR_LAB_0066e54c;
                    /* WARNING: Load size is inaccurate */
  FUN_006154a0((void *)(*(int *)(*this + 4) + (int)this),param_1);
  *(undefined4 *)((int)this + 4) = 0;
  ExceptionList = local_c;
  return this;
}

