
undefined4 __thiscall FUN_005f3090(void *this,int param_1)

{
  undefined4 uVar1;
  
  if (*(undefined **)((int)this + 4) != *(undefined **)(param_1 + 4)) {
    FUN_00618b60(*(undefined **)((int)this + 4));
    uVar1 = FUN_005f3650(*(char **)(param_1 + 4),(undefined4 *)((int)this + 4));
    *(undefined4 *)((int)this + 8) = uVar1;
  }
  return *(undefined4 *)((int)this + 8);
}

