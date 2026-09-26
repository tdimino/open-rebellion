
void __thiscall FUN_004be840(void *this,void *param_1,undefined4 param_2)

{
  int *piVar1;
  undefined4 uVar2;
  uint local_c;
  int iStack_8;
  undefined4 uStack_4;
  
  local_c = 0;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,(int)this + 0xc);
  FUN_005f4dd0(param_1,&iStack_8);
  while ((short)iStack_8 != 0) {
    FUN_005f4d90(param_1,&uStack_4);
    piVar1 = FUN_004bea20(this,uStack_4);
    if (piVar1 != (int *)0x0) {
      (**(code **)(*piVar1 + 0xc))(param_2);
      (**(code **)(*piVar1 + 4))(param_1);
      FUN_005f5440(this,piVar1);
    }
    iStack_8 = iStack_8 + 0xffff;
  }
  FUN_005f4d90(param_1,&local_c);
  uVar2 = FUN_005f5500(this,local_c);
  *(undefined4 *)((int)this + 0x10) = uVar2;
  FUN_005f4d90(param_1,&local_c);
  uVar2 = FUN_005f5500(this,local_c);
  *(undefined4 *)((int)this + 0x14) = uVar2;
  FUN_005f4d90(param_1,&local_c);
  uVar2 = FUN_005f5500(this,local_c);
  *(undefined4 *)((int)this + 0x18) = uVar2;
  return;
}

