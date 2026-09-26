
void __thiscall FUN_005d3650(void *this,undefined4 param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  int unaff_EBX;
  bool bVar4;
  bool bVar5;
  
  if (*(int *)((int)this + 0x38) == 0) {
    *(undefined4 *)((int)this + 0x40) = param_1;
  }
  else {
    if (*(int *)((int)this + 0x40) == 0) {
      piVar1 = *(int **)((int)this + 0x2c);
      uVar3 = *(undefined4 *)((int)this + 0x4c);
      iVar2 = *piVar1;
    }
    else {
      piVar1 = *(int **)((int)this + 0x2c);
      iVar2 = *piVar1;
      if (*(int *)((int)this + 0x40) == 1) {
        uVar3 = *(undefined4 *)((int)this + 0x48);
      }
      else {
        uVar3 = *(undefined4 *)((int)this + 0x44);
      }
    }
    iVar2 = (**(code **)(iVar2 + 0xa4))(piVar1,uVar3,0);
    FUN_005d8b00(iVar2);
    bVar4 = (*(byte *)((int)this + 0x120) & 2) != 0;
    if (bVar4) {
      (**(code **)(*(int *)this + 0x24))(2,0);
    }
    bVar5 = (*(byte *)((int)this + 0x120) & 1) != 0;
    if (bVar5) {
      (**(code **)(*(int *)this + 0x24))(1,0);
    }
    if (unaff_EBX == 0) {
      iVar2 = (**(code **)(**(int **)((int)this + 0x2c) + 0x48))
                        (*(int **)((int)this + 0x2c),*(undefined4 *)((int)this + 0x4c),0);
      FUN_005d8b00(iVar2);
      uVar3 = *(undefined4 *)((int)this + 0x4c);
    }
    else {
      piVar1 = *(int **)((int)this + 0x2c);
      if (unaff_EBX == 1) {
        iVar2 = (**(code **)(*piVar1 + 0x48))(piVar1,*(undefined4 *)((int)this + 0x48),0);
        FUN_005d8b00(iVar2);
        uVar3 = *(undefined4 *)((int)this + 0x48);
      }
      else {
        iVar2 = (**(code **)(*piVar1 + 0x48))(piVar1,*(undefined4 *)((int)this + 0x44));
        FUN_005d8b00(iVar2);
        uVar3 = *(undefined4 *)((int)this + 0x44);
      }
    }
    *(undefined4 *)((int)this + 0x34) = uVar3;
    *(int *)((int)this + 0x40) = unaff_EBX;
    if (bVar4) {
      (**(code **)(*(int *)this + 0x24))(2,1);
    }
    if (bVar5) {
      (**(code **)(*(int *)this + 0x24))(1,1);
      return;
    }
  }
  return;
}

