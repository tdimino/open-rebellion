
void __thiscall FUN_004397a0(void *this,void *param_1)

{
  uint *puVar1;
  uint uVar2;
  undefined4 local_4;
  
  uVar2 = 0;
  local_4 = 0;
  FUN_005f4db0(param_1,(int)this + 4);
  FUN_005f4db0(param_1,(int)this + 0x10);
  FUN_005f4db0(param_1,(int)this + 0x14);
  FUN_005f4db0(param_1,(int)this + 0xc);
  FUN_005f4db0(param_1,&DAT_006b28bc);
  FUN_005f4db0(param_1,&DAT_006b28c0);
  FUN_005f4e30(param_1,&DAT_006b28b0);
  FUN_005f4db0(param_1,&DAT_006b28c4);
  FUN_005f4db0(param_1,&DAT_006b28c8);
  FUN_0049c9f0((void *)((int)this + 0x3c),param_1);
  FUN_004e5210((void *)((int)this + 0x138),param_1);
  FUN_005f4db0(param_1,(int)this + 0x38);
  FUN_0049de40((void *)((int)this + 0x18),param_1);
  if (*(int *)((int)this + 0x34) != 0) {
    local_4 = *(undefined4 *)(*(int *)((int)this + 0x34) + 0x18);
  }
  FUN_005f4db0(param_1,&local_4);
  FUN_005f4e70(param_1,(int)this + 0x170);
  FUN_005f4db0(param_1,(int)this + 0x160);
  FUN_005f4db0(param_1,&DAT_006b28b8);
  puVar1 = (uint *)((int)this + 0x164);
  FUN_005f4db0(param_1,puVar1);
  FUN_005f4db0(param_1,(int)this + 0x188);
  FUN_005f4db0(param_1,(int)this + 0x18c);
  FUN_005f4db0(param_1,(int)this + 400);
  if ((*puVar1 != 0) && (*puVar1 != 0)) {
    do {
      FUN_005f4db0(param_1,*(int *)((int)this + 0x168) + uVar2 * 4);
      uVar2 = uVar2 + 1;
    } while (uVar2 < *puVar1);
  }
  FUN_005f4db0(param_1,(int)this + 0x148);
  FUN_005f4db0(param_1,(int)this + 0x14c);
  FUN_005f49a0((void *)((int)this + 0x150),param_1);
  FUN_005f5da0((void *)((int)this + 0x154),param_1);
  return;
}

