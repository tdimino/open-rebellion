
void __thiscall FUN_004c52d0(void *this,void *param_1)

{
  int *piVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar4;
  
  FUN_005f5610(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x1c);
  piVar1 = (int *)((int)this + 0x20);
  FUN_005f4d90(param_1,piVar1);
  FUN_005f4d90(param_1,(int)this + 0x24);
  uVar2 = FUN_00618b70(*piVar1 << 4);
  *(undefined4 *)((int)this + 0x28) = uVar2;
  uVar3 = 0;
  if (*piVar1 != 0) {
    iVar4 = 0;
    do {
      FUN_005f4dd0(param_1,*(int *)((int)this + 0x28) + iVar4);
      FUN_005f4d90(param_1,*(int *)((int)this + 0x28) + 4 + iVar4);
      FUN_005f4d90(param_1,*(int *)((int)this + 0x28) + 8 + iVar4);
      FUN_005f4d90(param_1,*(int *)((int)this + 0x28) + 0xc + iVar4);
      uVar3 = uVar3 + 1;
      iVar4 = iVar4 + 0x10;
    } while (uVar3 < *(uint *)((int)this + 0x20));
  }
  return;
}

