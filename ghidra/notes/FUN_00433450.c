
void * __thiscall FUN_00433450(void *this,uint param_1,uint param_2,uint param_3,void *param_4)

{
  undefined4 uVar1;
  bool bVar2;
  void *this_00;
  uint *puVar3;
  void *pvVar4;
  void *this_01;
  undefined4 *puVar5;
  undefined1 local_20 [4];
  undefined4 local_1c;
  void *local_18;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d1d5;
  local_c = ExceptionList;
  bVar2 = false;
  local_1c = 0;
  ExceptionList = &local_c;
  local_14 = this;
  local_18 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (local_18 == (void *)0x0) {
    param_4 = (void *)0x0;
  }
  else {
    param_4 = FUN_0041be80(local_18,param_4);
  }
  local_4 = 0xffffffff;
  if (param_4 != (void *)0x0) {
    for (this_00 = (void *)thunk_FUN_005f5060((int)this + 0x48); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      puVar3 = FUN_00403040(this_00,local_20);
      local_4 = 1;
      pvVar4 = FUN_004f5940(*(void **)((int)this + 0x40),puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (((pvVar4 != (void *)0x0) && ((param_1 & *(uint *)((int)pvVar4 + 0x24)) == param_1)) &&
         ((param_2 & *(uint *)((int)pvVar4 + 0x24)) == 0)) {
        this_01 = (void *)FUN_00618b70(0x18);
        local_4 = 2;
        local_10 = this_01;
        if (this_01 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          uVar1 = *(undefined4 *)((int)pvVar4 + (param_3 & 0xffff) * 4 + 0x44);
          puVar5 = FUN_00403040(pvVar4,&local_18);
          bVar2 = true;
          local_1c = 1;
          local_4 = CONCAT31(local_4._1_3_,3);
          pvVar4 = FUN_0041c360(this_01,puVar5,uVar1);
        }
        local_4 = 2;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_4,pvVar4);
        this = local_14;
      }
    }
    FUN_0041c230((int)param_4);
  }
  ExceptionList = local_c;
  return param_4;
}

