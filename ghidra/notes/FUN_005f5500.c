
void __thiscall FUN_005f5500(void *this,uint param_1)

{
  int iVar1;
  
  iVar1 = *(int *)((int)this + 4);
  while ((iVar1 != 0 && (param_1 != *(uint *)(iVar1 + 0x18)))) {
    if (*(uint *)(iVar1 + 0x18) < param_1) {
      iVar1 = *(int *)(iVar1 + 8);
    }
    else {
      iVar1 = *(int *)(iVar1 + 4);
    }
  }
  return;
}

