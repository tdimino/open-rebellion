
void __thiscall FUN_0043a0b0(void *this,uint param_1)

{
  bool bVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  
  uVar2 = param_1;
  if (((DAT_006b28b0 & 0x8000) == 0) && (bVar1 = false, *(int *)((int)this + 0x10) != 0)) {
    if ((*(uint *)((int)this + 0x150) & 0x10000000) == 0) {
      FUN_00439f20((int)this);
    }
    if ((param_1 != 0) && ((*(uint *)((int)this + 0x150) & param_1) == 0)) {
      param_1 = 0;
      *(uint *)((int)this + 0x150) = *(uint *)((int)this + 0x150) | uVar2;
      switch(uVar2) {
      case 1:
        param_1 = 6;
        break;
      case 2:
        param_1 = 2;
        break;
      case 4:
        param_1 = 3;
        break;
      case 8:
        param_1 = 5;
        break;
      case 0x10:
        param_1 = 4;
      }
      if (param_1 != 0) {
        puVar3 = (undefined4 *)thunk_FUN_005f5060((int)this + 0x154);
        while ((puVar3 != (undefined4 *)0x0 && (!bVar1))) {
          bVar1 = false;
          if ((puVar3[7] & 0xf) == param_1) {
            FUN_0048a590((void *)(*(int *)((int)this + 0x144) + 0x6c),(int)puVar3);
            bVar1 = true;
            uVar4 = thunk_FUN_004fcee0();
            *(undefined4 *)((int)this + 0x148) = uVar4;
            puVar3 = (undefined4 *)FUN_005f54d0((void *)((int)this + 0x154),puVar3[6]);
            if (puVar3 != (undefined4 *)0x0) {
              (**(code **)*puVar3)(1);
            }
          }
          puVar3 = (undefined4 *)puVar3[4];
        }
      }
    }
  }
  return;
}

