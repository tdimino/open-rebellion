
void * __thiscall FUN_00617760(void *this,int *param_1)

{
  int iVar1;
  uint uVar2;
  DWORD *pDVar3;
  byte local_10 [16];
  
  iVar1 = FUN_00617a90(this,0);
  if (iVar1 != 0) {
    uVar2 = FUN_00618510(this,(int)local_10);
    iVar1 = FUN_0061c520(local_10,(undefined4 *)0x0,uVar2);
    *param_1 = iVar1;
    if (iVar1 == -1) {
      pDVar3 = FUN_0061c250();
      if (*pDVar3 == 0x22) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
      }
    }
    iVar1 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
    if (*(int *)(iVar1 + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
    }
    if (*(int *)(*(int *)(*(int *)this + 4) + 0x34 + (int)this) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
    }
  }
  return this;
}

