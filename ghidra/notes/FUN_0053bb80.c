
undefined4 __thiscall FUN_0053bb80(void *this,int param_1,int param_2)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if ((((-1 < param_1) && (param_1 < 4)) && (-1 < param_2)) && (param_2 < 3)) {
    uVar1 = *(undefined4 *)((int)this + param_2 * 4 + param_1 * 0xc + 0x74);
  }
  return uVar1;
}

