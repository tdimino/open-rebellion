
void __thiscall FUN_00458780(void *this,uint param_1)

{
  uint uVar1;
  char *pcVar2;
  int iVar3;
  uint uVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  undefined4 auStack_38 [2];
  undefined4 auStack_30 [3];
  undefined4 auStack_24 [3];
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_006305c3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  SetFocus(*(HWND *)((int)this + 0x18));
  uVar1 = param_1;
  puVar6 = (undefined4 *)0x0;
  if (*(int *)((int)this + 0x170) != 0) {
    pcVar2 = (char *)FUN_00601b20(*(int *)(param_1 + 0x24));
    FUN_005f35b0(auStack_18,pcVar2);
    iStack_4 = 0;
    pcVar2 = (char *)FUN_00583c40(*(int *)((int)this + 0x170) + 0x98);
    FUN_005f35b0(auStack_30,pcVar2);
    iStack_4._0_1_ = 1;
    iVar3 = FUN_005f3390(auStack_18,(int)auStack_30);
    if (iVar3 != 0) {
      uVar4 = FUN_005f3070((int)auStack_30);
      if ((short)uVar4 != 0) {
        if (*(int *)((int)this + 0x268) == 0) {
          FUN_004ece30(&param_1);
          iStack_4._0_1_ = 2;
          pcVar2 = (char *)FUN_00583c40(*(int *)((int)this + 0x170) + 0x98);
          FUN_005f35b0(auStack_24,pcVar2);
          iStack_4._0_1_ = 3;
          FUN_004f4340(auStack_38);
          iStack_4._0_1_ = 4;
          puVar5 = (undefined4 *)FUN_00618b70(0x20);
          iStack_4._0_1_ = 5;
          if (puVar5 != (undefined4 *)0x0) {
            puVar6 = FUN_004f5ad0(puVar5);
          }
          iStack_4._0_1_ = 4;
          FUN_004f26d0(puVar6 + 6,(undefined4 *)(uVar1 + 0x14));
          FUN_004f57b0(auStack_38,puVar6);
          FUN_0041cdf0(0x215,(int)auStack_38,&param_1,(int)auStack_24);
          iStack_4._0_1_ = 3;
          FUN_004f4380(auStack_38);
          iStack_4._0_1_ = 2;
          FUN_005f2ff0(auStack_24);
          iStack_4._0_1_ = 1;
          FUN_00619730();
        }
        else {
          FUN_005f3090((void *)(*(int *)((int)this + 0x268) + 0x44),(int)auStack_30);
          FUN_0041ce20(*(int **)((int)this + 0x268),0);
          *(undefined4 *)((int)this + 0x268) = 0;
        }
      }
    }
    if (*(undefined4 **)((int)this + 0x268) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0x268))(1);
      *(undefined4 *)((int)this + 0x268) = 0;
    }
    DestroyWindow(*(HWND *)(*(int *)((int)this + 0x170) + 0x18));
    if (*(undefined4 **)((int)this + 0x170) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0x170))(1);
    }
    *(undefined4 *)((int)this + 0x170) = 0;
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_005f2ff0(auStack_30);
    iStack_4 = 0xffffffff;
    FUN_005f2ff0(auStack_18);
  }
  ExceptionList = pvStack_c;
  return;
}

