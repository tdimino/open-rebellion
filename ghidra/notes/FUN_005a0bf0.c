
void __thiscall FUN_005a0bf0(void *this,uint *param_1)

{
  int *piVar1;
  int iVar2;
  
  piVar1 = (int *)((int)this + 0x164);
  iVar2 = 8;
  do {
    *param_1 = (uint)(*piVar1 != 0);
    piVar1 = piVar1 + 1;
    param_1 = param_1 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  return;
}

