
void * __thiscall FUN_00617900(void *this,undefined2 *param_1)

{
  int iVar1;
  uint uVar2;
  byte local_10 [16];
  
  iVar1 = FUN_00617a90(this,0);
  if (iVar1 == 0) {
    return this;
  }
  uVar2 = FUN_00618510(this,(int)local_10);
  iVar1 = FUN_0061c270(local_10,(undefined4 *)0x0,uVar2);
  if (iVar1 < 0x8000) {
    if (-0x8001 < iVar1) {
      *param_1 = (short)iVar1;
      goto LAB_0061796a;
    }
    *param_1 = 0x8000;
  }
  else {
    *param_1 = 0x7fff;
  }
  *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
       *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
LAB_0061796a:
  iVar1 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  if (*(int *)((int)this + *(int *)(*(int *)this + 4) + 0x34) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
  }
  return this;
}

