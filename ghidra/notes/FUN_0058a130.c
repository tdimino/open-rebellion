
bool __thiscall FUN_0058a130(void *this,undefined4 param_1)

{
  int iVar1;
  bool bVar2;
  uint local_24 [2];
  undefined4 local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fc08;
  local_c = ExceptionList;
  bVar2 = true;
  if (*(int *)((int)this + 0x4c) == 0) {
    ExceptionList = &local_c;
    FUN_00589680(local_24,(void *)((int)this + 8));
    local_4 = 0;
    iVar1 = FUN_005875e0((void *)((int)this + 8),local_24,param_1);
    bVar2 = iVar1 != 0;
    if (bVar2) {
      *(undefined4 *)((int)this + 0x4c) = local_1c;
    }
    local_4 = 0xffffffff;
    FUN_005896d0(local_24);
  }
  ExceptionList = local_c;
  return bVar2;
}

