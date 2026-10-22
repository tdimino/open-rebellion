
void __thiscall FUN_005c7e10(void *this,int param_1)

{
  undefined4 uVar1;
  
  if (param_1 == 1) {
    if (*(int *)((int)this + 0x10) == 0) {
      *(undefined4 *)((int)this + 0x14) = 0;
    }
    uVar1 = FUN_005d0ae0();
    *(undefined4 *)((int)this + 0x18) = uVar1;
  }
  *(int *)((int)this + 8) = param_1;
  return;
}

