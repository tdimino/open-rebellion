
void __thiscall FUN_00616830(void *this,LPCSTR param_1,uint param_2,uint param_3)

{
  int iVar1;
  uint uVar2;
  void *pvVar3;
  
                    /* WARNING: Load size is inaccurate */
  pvVar3 = *(void **)(*(int *)(*this + 4) + 4 + (int)this);
  if ((*(int *)((int)pvVar3 + 0x4c) == -1) &&
     (pvVar3 = FUN_00618240(pvVar3,param_1,param_2 | 1,param_3), pvVar3 != (void *)0x0)) {
    return;
  }
                    /* WARNING: Load size is inaccurate */
  iVar1 = *(int *)(*this + 4);
  uVar2 = *(uint *)((int)this + iVar1 + 8);
  if (*(int *)((int)this + iVar1 + 0x34) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)this + iVar1 + 0x38));
  }
  *(uint *)((int)this + iVar1 + 8) = uVar2 | 2;
  if (*(int *)((int)this + iVar1 + 0x34) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)this + iVar1 + 0x38));
  }
  return;
}

