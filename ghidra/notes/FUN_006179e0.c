
void * __thiscall FUN_006179e0(void *this,int param_1,int param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006572ca;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e5d8;
    FUN_006153b0((undefined4 *)((int)this + 0xc));
    local_4 = 0;
  }
  *(undefined ***)(*(int *)(*(int *)this + 4) + (int)this) = &PTR_LAB_0066e5d4;
  FUN_006154a0((void *)(*(int *)(*(int *)this + 4) + (int)this),param_1);
  *(uint *)(*(int *)(*(int *)this + 4) + 0x24 + (int)this) =
       *(uint *)(*(int *)(*(int *)this + 4) + 0x24 + (int)this) | 1;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined4 *)((int)this + 4) = 0;
  ExceptionList = local_c;
  return this;
}

