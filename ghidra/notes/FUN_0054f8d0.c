
void * __thiscall FUN_0054f8d0(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064883b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 4) = *(undefined4 *)(param_1 + 4);
  *(undefined4 *)((int)this + 8) = *(undefined4 *)(param_1 + 8);
  FUN_00520580((void *)((int)this + 0xc),(undefined4 *)(param_1 + 0xc));
  local_4 = 0;
  FUN_00520580((void *)((int)this + 0x14),(undefined4 *)(param_1 + 0x14));
  *(undefined ***)this = &PTR_FUN_006620dc;
  ExceptionList = local_c;
  return this;
}

