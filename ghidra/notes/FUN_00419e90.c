
void * __thiscall
FUN_00419e90(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,void *param_7)

{
  undefined4 uVar1;
  bool bVar2;
  bool bVar3;
  void *this_00;
  void *pvVar4;
  undefined4 *puVar5;
  undefined1 local_18 [4];
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b30f;
  local_c = ExceptionList;
  bVar3 = false;
  bVar2 = false;
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
        if (*(int *)((int)this_00 + 0x3c) == 0) {
          pvVar4 = (void *)FUN_00618b70(0x18);
          local_4 = 3;
          local_10 = pvVar4;
          if (pvVar4 == (void *)0x0) {
            pvVar4 = (void *)0x0;
          }
          else {
            puVar5 = FUN_00403040(this_00,&local_14);
            bVar3 = true;
            local_4 = CONCAT31(local_4._1_3_,4);
            pvVar4 = FUN_0041c360(pvVar4,puVar5,100);
          }
          local_4 = 3;
          if (bVar3) {
            bVar3 = false;
            goto LAB_0041a017;
          }
        }
        else {
          pvVar4 = (void *)FUN_00618b70(0x18);
          local_4 = 1;
          local_10 = pvVar4;
          if (pvVar4 == (void *)0x0) {
            pvVar4 = (void *)0x0;
          }
          else {
            uVar1 = *(undefined4 *)((int)this_00 + 0x3c);
            puVar5 = FUN_00403040(this_00,local_18);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,2);
            pvVar4 = FUN_0041c360(pvVar4,puVar5,uVar1);
          }
          local_4 = 1;
          if (bVar2) {
            bVar2 = false;
LAB_0041a017:
            FUN_00619730();
          }
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_7,pvVar4);
      }
    }
    FUN_0041c230((int)param_7);
  }
  ExceptionList = local_c;
  return param_7;
}

