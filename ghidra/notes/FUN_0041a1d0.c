
void * __thiscall
FUN_0041a1d0(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,void *param_7)

{
  bool bVar1;
  void *this_00;
  void *pvVar2;
  undefined4 *puVar3;
  int iVar4;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b38d;
  local_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &local_c;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (local_14 == (void *)0x0) {
    param_7 = (void *)0x0;
  }
  else {
    param_7 = FUN_0041be80(local_14,param_7);
  }
  local_4 = 0xffffffff;
  if (param_7 != (void *)0x0) {
    for (this_00 = (void *)thunk_FUN_005f5060((int)this + 0x44); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      if (((((param_1 & *(uint *)((int)this_00 + 0x24)) == param_1) &&
           ((param_4 & *(uint *)((int)this_00 + 0x24)) == 0)) &&
          ((param_2 & *(uint *)((int)this_00 + 0x28)) == param_2)) &&
         ((((param_5 & *(uint *)((int)this_00 + 0x28)) == 0 &&
           ((param_3 & *(uint *)((int)this_00 + 0x2c)) == param_3)) &&
          ((param_6 & *(uint *)((int)this_00 + 0x2c)) == 0)))) {
        pvVar2 = (void *)FUN_00618b70(0x18);
        local_4 = 1;
        local_10 = pvVar2;
        if (pvVar2 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          puVar3 = FUN_00403040(this_00,&local_14);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,2);
          iVar4 = FUN_0041cd80(0x14);
          pvVar2 = FUN_0041c360(pvVar2,puVar3,iVar4);
        }
        local_4 = 1;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_7,pvVar2);
      }
    }
    FUN_0041c230((int)param_7);
  }
  ExceptionList = local_c;
  return param_7;
}

