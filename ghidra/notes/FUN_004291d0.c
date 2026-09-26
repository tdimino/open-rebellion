
int __thiscall FUN_004291d0(void *this,int param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = 0;
  piVar2 = (int *)((int)this + 0x2f0);
  do {
    if (*piVar2 == param_1) {
      return iVar1 + 1;
    }
    iVar1 = iVar1 + 1;
    piVar2 = piVar2 + 7;
  } while (iVar1 < 0xc);
  return 0;
}

