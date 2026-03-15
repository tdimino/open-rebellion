
void * __thiscall FUN_005f3ef0(void *this,void *param_1)

{
  uint uVar1;
  int iVar2;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065565b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f3dc0(local_18,(int)this);
  uVar1 = *(uint *)((int)this + 4);
  local_4 = 1;
  if ((uVar1 != 0) && (*(uint *)((int)this + 8) != uVar1)) {
    if (*(uint *)((int)this + 8) < uVar1) {
      iVar2 = uVar1 - 1;
    }
    else {
      iVar2 = 0;
    }
    *(int *)((int)this + 4) = iVar2;
  }
  FUN_005f3dc0(param_1,(int)local_18);
  local_4 = local_4 & 0xffffff00;
  FUN_005f3e00(local_18);
  ExceptionList = local_c;
  return param_1;
}

