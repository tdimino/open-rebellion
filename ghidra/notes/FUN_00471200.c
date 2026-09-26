
void __thiscall FUN_00471200(void *this,HDC param_1)

{
  undefined4 *this_00;
  int iVar1;
  uint uVar2;
  int iVar3;
  
  if (*(int *)((int)this + 0x188) != 0) {
    FUN_00470e70((int)this);
  }
  if ((0x6f < *(uint *)((int)this + 0x17c)) && (*(uint *)((int)this + 0x17c) < 0x72)) {
    iVar1 = *(int *)(*(int *)((int)this + 0x160) + 0x94);
    if (iVar1 == 0) {
      uVar2 = 0;
    }
    else {
      uVar2 = *(uint *)(iVar1 + 0x24);
    }
    iVar1 = FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),uVar2);
    this_00 = (undefined4 *)((int)this + 0x128);
    *(undefined4 *)((int)this + 0x154) = 1;
    FUN_00601b30(this_00,0x25,99);
    *this_00 = 0x15b;
    FUN_00403e90(this_00,0);
    FUN_00601c60(this_00,4);
    FUN_00601c90(this_00,0xffffff,(HWND)0x0);
    FUN_00601aa0(this_00,*(char **)(iVar1 + 0x84));
    FUN_00601ce0(this_00,param_1);
    iVar1 = *(int *)(*(int *)((int)this + 0x15c) + 0x94);
    if (iVar1 == 0) {
      uVar2 = 0;
    }
    else {
      uVar2 = *(uint *)(iVar1 + 0x24);
    }
    iVar1 = FUN_00604500((void *)(*(int *)((int)this + 0x15c) + 0x6c),uVar2);
    FUN_00601b30(this_00,0xc,0x28);
    *this_00 = 400;
    FUN_00403e90(this_00,1);
    FUN_00601c60(this_00,6);
    iVar3 = FUN_00602510(iVar1);
    if (iVar3 == 0x70) {
      iVar3 = 0xff;
    }
    else if (iVar3 == 0x71) {
      iVar3 = 0xff00;
    }
    else {
      iVar3 = 0xffffff;
    }
    FUN_00601c90(this_00,iVar3,(HWND)0x0);
    FUN_00601aa0(this_00,*(char **)(iVar1 + 0x84));
    FUN_00601ce0(this_00,param_1);
  }
  return;
}

