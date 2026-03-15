
void * __thiscall
FUN_00419a00(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,uint param_7,void *param_8)

{
  undefined4 uVar1;
  bool bVar2;
  void *this_00;
  void *pvVar3;
  undefined4 *puVar4;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b23d;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (local_14 == (void *)0x0) {
    param_8 = (void *)0x0;
  }
  else {
    param_8 = FUN_0041be80(local_14,param_8);
  }
  local_4 = 0xffffffff;
  if (param_8 != (void *)0x0) {
    for (this_00 = (void *)thunk_FUN_005f5060((int)this + 0x44); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      if (((((param_1 & *(uint *)((int)this_00 + 0x24)) == param_1) &&
           ((param_4 & *(uint *)((int)this_00 + 0x24)) == 0)) &&
          ((param_2 & *(uint *)((int)this_00 + 0x28)) == param_2)) &&
         ((((param_5 & *(uint *)((int)this_00 + 0x28)) == 0 &&
           ((param_3 & *(uint *)((int)this_00 + 0x2c)) == param_3)) &&
          ((param_6 & *(uint *)((int)this_00 + 0x2c)) == 0)))) {
        pvVar3 = (void *)FUN_00618b70(0x18);
        local_4 = 1;
        local_10 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          uVar1 = *(undefined4 *)((int)this_00 + (param_7 & 0xffff) * 4 + 0x70);
          puVar4 = (undefined4 *)FUN_00403040(this_00,&local_14);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar3 = FUN_0041c360(pvVar3,puVar4,uVar1);
        }
        local_4 = 1;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_8,pvVar3);
      }
    }
    FUN_0041c230((int)param_8);
  }
  ExceptionList = local_c;
  return param_8;
}

