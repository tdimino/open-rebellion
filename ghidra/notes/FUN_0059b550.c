
void __thiscall FUN_0059b550(void *this,int *param_1,uint param_2)

{
  void *this_00;
  undefined4 *puVar1;
  uint uVar2;
  
  uVar2 = 0;
  puVar1 = (undefined4 *)((int)this + 0xd8);
  do {
    this_00 = (void *)puVar1[-8];
    if (*param_1 == 0) {
LAB_0059b593:
      if (*(int *)((int)this_00 + 0x11c) != 0) {
        FUN_00600c40(this,this_00,&DAT_006bc398);
        FUN_005c54d0(this_00);
      }
    }
    else {
      if (*(int *)((int)this_00 + 0x11c) == 0) {
        FUN_00600c40(this,this_00,*puVar1);
        FUN_005c5480(this_00);
      }
      if (*param_1 == 0) goto LAB_0059b593;
    }
    if ((param_2 < 8) && (*param_1 != 0)) {
      if (param_2 == uVar2) {
        FUN_006030c0(*(void **)((int)this + param_2 * 4 + 0xb8),5);
      }
      else {
        FUN_006030f0(this_00,5);
      }
    }
    uVar2 = uVar2 + 1;
    puVar1 = puVar1 + 1;
    param_1 = param_1 + 1;
    if (7 < uVar2) {
      return;
    }
  } while( true );
}

