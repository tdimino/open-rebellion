
void __thiscall FUN_005d4a60(void *this,undefined4 param_1)

{
  int iVar1;
  int *piVar2;
  undefined4 uVar3;
  int unaff_ESI;
  
  if (*(int *)((int)this + 0x38) != 0) {
    if (*(int *)((int)this + 0x4c) == 0) {
      piVar2 = *(int **)((int)this + 0x2c);
      uVar3 = *(undefined4 *)((int)this + 0x40);
      iVar1 = *piVar2;
    }
    else {
      piVar2 = *(int **)((int)this + 0x2c);
      iVar1 = *piVar2;
      if (*(int *)((int)this + 0x4c) == 1) {
        uVar3 = *(undefined4 *)((int)this + 0x44);
      }
      else {
        uVar3 = *(undefined4 *)((int)this + 0x48);
      }
    }
    iVar1 = (**(code **)(iVar1 + 0xa4))(piVar2,uVar3,0);
    FUN_005d8b00(iVar1);
    if (unaff_ESI == 0) {
      piVar2 = *(int **)((int)this + 0x2c);
      uVar3 = *(undefined4 *)((int)this + 0x40);
      iVar1 = *piVar2;
    }
    else {
      piVar2 = *(int **)((int)this + 0x2c);
      iVar1 = *piVar2;
      if (unaff_ESI == 1) {
        uVar3 = *(undefined4 *)((int)this + 0x44);
      }
      else {
        uVar3 = *(undefined4 *)((int)this + 0x48);
      }
    }
    iVar1 = (**(code **)(iVar1 + 0x48))(piVar2,uVar3,0);
    FUN_005d8b00(iVar1);
    *(int *)((int)this + 0x4c) = unaff_ESI;
    return;
  }
  *(undefined4 *)((int)this + 0x4c) = param_1;
  return;
}

