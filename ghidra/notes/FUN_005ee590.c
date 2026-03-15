
/* WARNING: Removing unreachable block (ram,0x005ee93b) */
/* WARNING: Removing unreachable block (ram,0x005ee923) */
/* WARNING: Removing unreachable block (ram,0x005ee926) */
/* WARNING: Removing unreachable block (ram,0x005ee92c) */
/* WARNING: Removing unreachable block (ram,0x005ee967) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall
FUN_005ee590(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4)

{
  undefined4 *puVar1;
  int iVar2;
  undefined4 uVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655453;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005d8980(this,0,param_1);
  local_4 = 0;
  FUN_005fbb20((undefined4 *)((int)this + 0x40));
  *(undefined4 *)((int)this + 0x5c) = 0;
  *(undefined4 *)((int)this + 0x60) = 0;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x68) = 0xbf800000;
  *(undefined4 *)((int)this + 0x70) = param_3;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0066db48;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066db38;
  *(undefined4 *)((int)this + 0x34) = 0;
  puVar1 = (undefined4 *)((int)this + 0x38);
  iVar2 = (**(code **)(*DAT_006bcbd0 + 0x18))();
  FUN_005d8b00(iVar2);
  if (param_4 == 2) {
    iVar2 = *(int *)*puVar1;
  }
  else {
    iVar2 = *(int *)*puVar1;
  }
  iVar2 = (**(code **)(iVar2 + 0x5c))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(*(int *)*puVar1 + 0x78))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(*(int *)*puVar1 + 0x34))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(*(int *)*puVar1 + 100))((int *)*puVar1);
  FUN_005d8b00(iVar2);
  *(float *)((int)this + 0x58) = _DAT_0066db10 * _DAT_0066db04;
  uVar3 = 0;
  iVar2 = (**(code **)(**(int **)((int)this + 0x2c) + 0x48))(*(int **)((int)this + 0x2c),*puVar1);
  FUN_005d8b00(iVar2);
  (**(code **)(*(int *)*puVar1 + 8))((int *)*puVar1);
  *(uint *)((int)this + 0x6c) = CONCAT31(uRam00000030,piRam0000002c._3_1_);
  (**(code **)(*piRam0000002c + 0x60))(piRam0000002c,param_1,&stack0xfffffe50);
  iVar2 = (**(code **)(**(int **)((int)this + 0x2c) + 0xf4))(*(int **)((int)this + 0x2c),param_1);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x2c) + 0x90))
                    (*(int **)((int)this + 0x2c),*(undefined4 *)((int)this + 0x6c),param_1,0,0);
  FUN_005d8b00(iVar2);
  *(undefined4 *)((int)this + 0x60) = 0;
  *(undefined4 *)((int)this + 0x5c) = uVar3;
  *(undefined4 *)((int)this + 100) = 0x3f800000;
  *(undefined4 *)((int)this + 0x68) = 0xbf800000;
  FUN_005fbbb0((int)this + 0x40);
  iVar2 = (**(code **)(**(int **)((int)this + 0x2c) + 0x34))
                    (*(int **)((int)this + 0x2c),&LAB_005eeb90,this,0);
  FUN_005d8b00(iVar2);
  ExceptionList = (void *)0x3a83126f;
  return this;
}

