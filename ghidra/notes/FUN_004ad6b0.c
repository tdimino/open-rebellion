
void * __thiscall FUN_004ad6b0(void *this,int param_1)

{
  char *pcVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638788;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060ba00(this,*(undefined4 *)(param_1 + 0x18));
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065c090;
  *(int *)((int)this + 0x68) = param_1;
  pcVar1 = (char *)FUN_00583c40(param_1 + 0x44);
  FUN_005f35e0((void *)((int)this + 0x14),pcVar1);
  *(undefined4 *)((int)this + 0x6c) = 1;
  ExceptionList = local_c;
  return this;
}

