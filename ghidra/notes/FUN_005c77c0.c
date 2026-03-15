
void __thiscall FUN_005c77c0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  int local_18;
  undefined4 local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00653d4b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f4d90(param_1,(int)this + 0x14);
  FUN_005f4d90(pvVar1,(int)this + 0x18);
  FUN_005f4d90(pvVar1,(int)this + 0x1c);
  FUN_005f4d90(pvVar1,(int)this + 0x24);
  if (*(undefined4 **)((int)this + 0x2c) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)((int)this + 0x2c))(1);
  }
  *(undefined4 *)((int)this + 0x2c) = 0;
  FUN_005f4d90(pvVar1,&local_18);
  *(int *)((int)this + 0x28) = local_18;
  if (local_18 == 3) {
    puVar2 = (undefined4 *)FUN_00618b70(4);
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      *puVar2 = &PTR_FUN_0066cc10;
    }
    *(undefined4 **)((int)this + 0x2c) = puVar2;
  }
  if (*(int *)((int)this + 0x28) == 2) {
    puVar2 = (undefined4 *)FUN_00618b70(4);
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      *puVar2 = &PTR_FUN_0066cbf8;
    }
    *(undefined4 **)((int)this + 0x2c) = puVar2;
  }
  if (*(undefined4 **)((int)this + 0x34) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)((int)this + 0x34))(1);
  }
  *(undefined4 *)((int)this + 0x34) = 0;
  FUN_005f4d90(pvVar1,&local_18);
  *(int *)((int)this + 0x30) = local_18;
  if (local_18 == 1) {
    puVar2 = (undefined4 *)FUN_00618b70(4);
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      *puVar2 = &PTR_FUN_0066cbf0;
    }
    *(undefined4 **)((int)this + 0x34) = puVar2;
  }
  if (*(int *)((int)this + 0x30) == 2) {
    puVar2 = (undefined4 *)FUN_00618b70(4);
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      *puVar2 = &PTR_FUN_0066cbe8;
    }
    *(undefined4 **)((int)this + 0x34) = puVar2;
  }
  FUN_005f4d90(pvVar1,(int)this + 0x38);
  FUN_005f4d90(pvVar1,(int)this + 0x3c);
  FUN_005f4d90(pvVar1,(int)this + 0x40);
  FUN_005f4d90(pvVar1,(int)this + 0x44);
  FUN_005f4d90(pvVar1,(int)this + 0x20);
  FUN_005cbe30((void *)((int)this + 0x48),pvVar1);
  FUN_005c5de0((void *)((int)this + 0x58),pvVar1);
  FUN_005c5de0((void *)((int)this + 100),pvVar1);
  FUN_005cbab0((void *)((int)this + 0x70),pvVar1);
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 8) = param_1;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x10) = param_1;
  FUN_005f4d90(pvVar1,(int)this + 0x7c);
  FUN_005f4d90(pvVar1,(int)this + 0x80);
  local_14 = 0;
  FUN_005f4d90(pvVar1,&local_14);
  *(undefined4 *)((int)this + 0x84) = local_14;
  (**(code **)(*(int *)((int)this + 0x88) + 4))();
  param_1 = (void *)0x0;
  FUN_005f4d90(pvVar1,&param_1);
  for (; 0 < (int)param_1; param_1 = (void *)((int)param_1 + -1)) {
    puVar2 = (undefined4 *)FUN_00618b70(0x30);
    uStack_4 = 0;
    puStack_10 = puVar2;
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      FUN_005f5c10(puVar2);
      puVar2[7] = &PTR_LAB_0066c2c0;
      *puVar2 = &PTR_FUN_0066cbe0;
    }
    uStack_4 = 0xffffffff;
    FUN_005f4d90(pvVar1,puVar2 + 6);
    (**(code **)(puVar2[7] + 4))(pvVar1);
    FUN_005f4f10((int *)((int)this + 0x88),(int)puVar2);
  }
  ExceptionList = pvStack_c;
  return;
}

