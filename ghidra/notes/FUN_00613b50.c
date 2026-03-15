
undefined4 __thiscall FUN_00613b50(void *this,int param_1)

{
  undefined1 uVar1;
  undefined4 uVar2;
  
  uVar2 = 0x80004005;
  if (*(int *)((int)this + 0x4c) != 0) {
    return uVar2;
  }
  if (*(int *)((int)this + 0x34) != 0) {
    FUN_00613c00((int)this);
    uVar1 = 1;
    *(int *)((int)this + 0x74) = param_1;
    if ((*(int *)((int)this + 0x54) != 1) && (param_1 == 0)) {
      uVar1 = 0;
    }
    if (*(int *)((int)this + 0xbc) == 0) {
      uVar2 = FUN_00613ff0();
    }
    else {
      uVar2 = *(undefined4 *)((int)this + 0xc0);
    }
    (**(code **)(**(int **)((int)this + 0x34) + 0x3c))(*(int **)((int)this + 0x34),uVar2);
    uVar2 = (**(code **)(**(int **)((int)this + 0x34) + 0x30))
                      (*(int **)((int)this + 0x34),0,0,uVar1);
  }
  return uVar2;
}

