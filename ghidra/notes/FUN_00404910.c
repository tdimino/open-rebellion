
bool __thiscall FUN_00404910(void *this,int *param_1,undefined4 *param_2)

{
  int iVar1;
  bool bVar2;
  
  *param_2 = 0;
  bVar2 = *(int *)((int)this + 0x54) == 0;
  if (!bVar2) {
    *(int *)((int)this + 0x54) = *(int *)((int)this + 0x54) + -1;
  }
  if (0 < *(int *)((int)this + 0x58)) {
    *(int *)((int)this + 0x58) = *(int *)((int)this + 0x58) + -1;
    *param_2 = 1;
    *param_1 = *(int *)(*(int *)((int)this + 0x10) + 0x28);
    if ((int *)**(undefined4 **)((int)this + 0x4c) == (int *)0x0) {
      iVar1 = 0;
    }
    else {
      iVar1 = FUN_005fc0e0((int *)**(undefined4 **)((int)this + 0x4c));
    }
    param_1[2] = *param_1 + iVar1;
    param_1[1] = *(int *)(*(int *)((int)this + 0x10) + 0x2c);
    if ((int *)**(undefined4 **)((int)this + 0x4c) != (int *)0x0) {
      iVar1 = FUN_005fc0f0((int *)**(undefined4 **)((int)this + 0x4c));
      param_1[3] = param_1[1] + iVar1;
      return bVar2;
    }
    param_1[3] = param_1[1];
  }
  return bVar2;
}

