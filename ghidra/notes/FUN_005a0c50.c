
int __thiscall FUN_005a0c50(void *this,int param_1)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  
  if (param_1 != 0) {
    iVar1 = *(int *)(param_1 + 0x178);
    if (iVar1 != 0) {
      uVar3 = 0;
      piVar2 = (int *)((int)this + 0x164);
      while (*piVar2 != iVar1) {
        uVar3 = uVar3 + 1;
        piVar2 = piVar2 + 1;
        if (7 < uVar3) {
          return iVar1;
        }
      }
    }
  }
  return 0;
}

