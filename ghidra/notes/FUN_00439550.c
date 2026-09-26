
void __thiscall FUN_00439550(void *this,void *param_1,undefined4 param_2)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  uint uVar4;
  uint local_4;
  
  local_4 = 0;
  *(undefined4 *)((int)this + 0x144) = param_2;
  FUN_005f4d90(param_1,(int)this + 4);
  FUN_005f4d90(param_1,(int)this + 0x10);
  FUN_005f4d90(param_1,(int)this + 0x14);
  FUN_005f4d90(param_1,(int)this + 0xc);
  FUN_005f4d90(param_1,&DAT_006b28bc);
  FUN_005f4d90(param_1,&DAT_006b28c0);
  FUN_005f4e10(param_1,&DAT_006b28b0);
  FUN_005f4d90(param_1,&DAT_006b28c4);
  FUN_005f4d90(param_1,&DAT_006b28c8);
  FUN_0049c8a0((void *)((int)this + 0x3c),param_1);
  FUN_0049c6f0((void *)((int)this + 0x138),param_1);
  FUN_005f4d90(param_1,(int)this + 0x38);
  FUN_0049dd50((void *)((int)this + 0x18),param_1,(void *)((int)this + 0x3c));
  FUN_005f4d90(param_1,&local_4);
  uVar2 = FUN_005f5500((void *)((int)this + 0x18),local_4);
  *(undefined4 *)((int)this + 0x34) = uVar2;
  FUN_005f4e50(param_1,(int)this + 0x170);
  FUN_005f4d90(param_1,(int)this + 0x160);
  FUN_005f4d90(param_1,&DAT_006b28b8);
  piVar1 = (int *)((int)this + 0x164);
  FUN_005f4d90(param_1,piVar1);
  FUN_005f4d90(param_1,(int)this + 0x188);
  FUN_005f4d90(param_1,(int)this + 0x18c);
  FUN_005f4d90(param_1,(int)this + 400);
  if (*(undefined **)((int)this + 0x168) != (undefined *)0x0) {
    FUN_00618b60(*(undefined **)((int)this + 0x168));
  }
  if (*(undefined **)((int)this + 0x16c) != (undefined *)0x0) {
    FUN_00618b60(*(undefined **)((int)this + 0x16c));
  }
  if (*piVar1 != 0) {
    uVar2 = FUN_00618b70(*piVar1 * 4);
    *(undefined4 *)((int)this + 0x168) = uVar2;
    uVar2 = FUN_00618b70(*piVar1 << 2);
    *(undefined4 *)((int)this + 0x16c) = uVar2;
    uVar4 = 0;
    if (*piVar1 != 0) {
      do {
        iVar3 = uVar4 * 4;
        FUN_005f4d90(param_1,iVar3 + *(int *)((int)this + 0x168));
        uVar4 = uVar4 + 1;
        *(undefined4 *)(iVar3 + *(int *)((int)this + 0x16c)) = 0;
      } while (uVar4 < *(uint *)((int)this + 0x164));
    }
  }
  FUN_005f4d90(param_1,(int)this + 0x148);
  FUN_005f4d90(param_1,(int)this + 0x14c);
  FUN_005f4990((void *)((int)this + 0x150),param_1);
  (**(code **)(*(int *)((int)this + 0x154) + 4))();
  FUN_005f5cf0((void *)((int)this + 0x154),param_1);
  iVar3 = FUN_005f5500((void *)((int)this + 0x18),0x17);
  *(int *)((int)this + 0x184) = iVar3;
  *(void **)(iVar3 + 0x54) = this;
  return;
}

