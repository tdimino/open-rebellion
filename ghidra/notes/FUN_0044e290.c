
void __thiscall FUN_0044e290(int param_1,HDC param_2)

{
  undefined4 *this;
  int iVar1;
  int iVar2;
  uint uVar3;
  
  if ((0x65 < *(uint *)(param_1 + 0x17c)) && (*(uint *)(param_1 + 0x17c) < 0x68)) {
    iVar2 = *(int *)(*(int *)(param_1 + 0x15c) + 0x94);
    if (iVar2 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(iVar2 + 0x24);
    }
    iVar1 = FUN_00604500((void *)(*(int *)(param_1 + 0x15c) + 0x6c),uVar3);
    iVar2 = *(int *)(*(int *)(param_1 + 0x174) + 0x6c);
    if (iVar2 == 1) {
      iVar2 = 0x20000ff;
    }
    else if (iVar2 == 2) {
      iVar2 = 0x200ff00;
    }
    else {
      iVar2 = 0x2ffff00;
    }
    this = (undefined4 *)(param_1 + 0x128);
    FUN_00601c90(this,iVar2,(HWND)0x0);
    FUN_00601b30(this,0xc,0x24);
    *(undefined4 *)(param_1 + 0x154) = 1;
    *this = 0x15b;
    FUN_00403e90(this,1);
    FUN_00601c60(this,6);
    FUN_00601aa0(this,*(char **)(iVar1 + 0x84));
    FUN_00601b80(this,param_2);
    FUN_00601ce0(this,param_2);
    iVar2 = *(int *)(*(int *)(param_1 + 0x160) + 0x94);
    if (iVar2 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(iVar2 + 0x24);
    }
    iVar2 = FUN_00604500((void *)(*(int *)(param_1 + 0x160) + 0x6c),uVar3);
    FUN_00601c90(this,0x2ffffff,(HWND)0x0);
    FUN_00601b30(this,0x26,100);
    *this = 400;
    FUN_00403e90(this,0);
    FUN_00601c60(this,4);
    FUN_00601aa0(this,*(char **)(iVar2 + 0x84));
    FUN_00601b80(this,param_2);
    FUN_00601ce0(this,param_2);
  }
  return;
}

