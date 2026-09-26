
void __thiscall FUN_004632d0(void *this,int param_1)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  
  if (*(int *)((int)this + 0x148) == param_1) {
    return;
  }
  FUN_00604f90(*(void **)((int)this + 0x150),(char *)0x0);
  SetCaretPos(0,0);
  *(int *)((int)this + 0x148) = param_1;
  if (param_1 == 1) {
    iVar2 = 1;
  }
  else {
    if (param_1 != 2) goto LAB_00463312;
    iVar2 = 2;
  }
  FUN_006075e0(this,iVar2);
LAB_00463312:
  uVar3 = *(uint *)(*(int *)((int)this + 0x154) + 0x2c);
  iVar2 = *(int *)(*(int *)((int)this + 0x154) + 0x28);
  puVar1 = (uint *)FUN_006073d0(this);
  FUN_00608300(*(void **)((int)this + 0x154),puVar1,iVar2,uVar3);
  uVar3 = *(uint *)((int)this + 0x160);
  *(uint *)((int)this + 0x164) = uVar3;
  *(undefined4 *)((int)this + 0x160) = 0;
  FUN_00462be0(this,uVar3);
  return;
}

