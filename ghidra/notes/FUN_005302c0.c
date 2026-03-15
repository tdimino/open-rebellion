
uint __thiscall FUN_005302c0(void *this,int *param_1,int *param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 *puVar3;
  int extraout_ECX;
  undefined4 *puVar4;
  int iStack_14;
  
  *param_2 = 0;
  iStack_14 = 0x5302d6;
  uVar2 = FUN_00533080((int)this);
  puVar4 = (undefined4 *)0x0;
  iStack_14 = 0x5302e5;
  puVar3 = (undefined4 *)thunk_FUN_005f5060(*(int *)((int)this + 200));
  iStack_14 = extraout_ECX;
  if (uVar2 != 0) {
    while ((puVar3 != (undefined4 *)0x0 && (puVar4 == (undefined4 *)0x0))) {
      iStack_14 = puVar3[9];
      if (iStack_14 == *param_1) {
        puVar4 = puVar3;
      }
      puVar3 = (undefined4 *)puVar3[4];
    }
  }
  if (uVar2 != 0) {
    if (puVar4 == (undefined4 *)0x0) {
      FUN_004f26d0(&iStack_14,param_1);
      puVar4 = FUN_004f42a0(*(void **)((int)this + 200));
      uVar2 = (uint)(puVar4 != (undefined4 *)0x0);
    }
    if (uVar2 != 0) {
      iVar1 = puVar4[8];
      *param_2 = iVar1 + 1;
      puVar4[8] = iVar1 + 1;
    }
  }
  return uVar2;
}

