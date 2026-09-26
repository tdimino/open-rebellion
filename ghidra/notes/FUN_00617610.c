
void * __thiscall FUN_00617610(void *this,undefined4 *param_1)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  DWORD *pDVar4;
  byte local_10 [16];
  
  iVar1 = FUN_00617a90(this,0);
  if (iVar1 != 0) {
    uVar2 = FUN_00618510(this,(int)local_10);
    uVar3 = FUN_0061c270(local_10,(undefined4 *)0x0,uVar2);
    *param_1 = uVar3;
    pDVar4 = FUN_0061c250();
    if (*pDVar4 == 0x22) {
      *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
           *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
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

