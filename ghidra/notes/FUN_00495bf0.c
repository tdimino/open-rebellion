
undefined4 __thiscall FUN_00495bf0(void *this,int param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  char *pcVar5;
  undefined4 uVar6;
  undefined4 uStack_18;
  undefined4 uStack_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00636148;
  local_c = ExceptionList;
  uVar6 = 1;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x28) = 0x14;
  if (param_1 == 0) {
    uVar6 = 0;
  }
  else {
    iVar2 = FUN_00495d00(param_1);
    *(int *)((int)this + 0x78) = iVar2;
    for (piVar4 = *(int **)(param_1 + 8); piVar4 != (int *)0x0; piVar4 = (int *)piVar4[2]) {
      (**(code **)(*piVar4 + 0x1c))(this);
    }
    if (*(int *)((int)this + 0x40) != 0) {
      if (*(short *)((int)this + 0x2c) == 0) {
        uVar3 = *(uint *)((int)this + 0x74) >> 0x18;
        uStack_18 = 0x3c;
        uStack_14 = 0x40;
        if ((uVar3 < 0x3c) || (0x3f < uVar3)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if ((bVar1) &&
           (piVar4 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)((int)this + 0x74)),
           piVar4 != (int *)0x0)) {
          uVar3 = FUN_004c5000(piVar4);
          *(short *)((int)this + 0x2e) = (short)uVar3;
        }
      }
      pcVar5 = (char *)FUN_00583c40((int)this + 0x68);
      FUN_005f35b0(&uStack_18,pcVar5);
      uStack_4 = 0;
      FUN_005f30d0((void *)((int)this + 0x50),(int)&uStack_18);
      uStack_4 = 0xffffffff;
      FUN_005f2ff0(&uStack_18);
    }
  }
  ExceptionList = local_c;
  return uVar6;
}

