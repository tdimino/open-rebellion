
uint __thiscall FUN_006138b0(void *this,int *param_1)

{
  int *this_00;
  undefined1 *puVar1;
  uint uVar2;
  undefined1 *puVar3;
  int iVar4;
  undefined4 unaff_ESI;
  uint unaff_EDI;
  undefined1 *puStack_30;
  undefined1 *puStack_2c;
  undefined1 *puStack_28;
  int **ppiStack_24;
  undefined4 uStack_20;
  undefined1 local_c [4];
  undefined1 local_8 [4];
  undefined1 local_4 [4];
  
  this_00 = param_1;
  uVar2 = *(uint *)((int)this + 0x34);
  if ((((uVar2 != 0) && (param_1 != (int *)0x0)) &&
      (uVar2 = *(int *)(*param_1 + 4) + (int)param_1,
      *(int *)(*(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 0x4c) != -1)) &&
     ((*(byte *)(uVar2 + 8) & 1) == 0)) {
    uStack_20 = 0x6138f7;
    puVar1 = (undefined1 *)FUN_00613830((int)this);
    uStack_20 = *(undefined4 *)((int)this + 0x5c);
    ppiStack_24 = &param_1;
    puStack_28 = local_8;
    puStack_2c = local_c;
    puStack_30 = local_4;
    uVar2 = (**(code **)(**(int **)((int)this + 0x34) + 0x2c))
                      (*(int **)((int)this + 0x34),*(undefined4 *)((int)this + 0x58),puVar1);
    if (uVar2 == 0x88780096) {
      (**(code **)(**(int **)((int)this + 0x34) + 0x50))(*(int **)((int)this + 0x34));
      uVar2 = (**(code **)(**(int **)((int)this + 0x34) + 0x2c))
                        (*(int **)((int)this + 0x34),*(undefined4 *)((int)this + 0x58),puVar1,
                         &puStack_28,&puStack_30,&puStack_2c,&uStack_20,
                         *(undefined4 *)((int)this + 0x5c));
    }
    if (uVar2 == 0) {
      puVar3 = *(undefined1 **)((int)this + 100);
      if (puStack_2c < *(undefined1 **)((int)this + 100)) {
        puVar3 = puStack_2c;
      }
      FUN_006173b0(this_00,ppiStack_24,(uint)puVar3);
      puVar3 = (undefined1 *)this_00[2];
      uVar2 = *(int *)((int)this + 100) - (int)puVar3;
      *(uint *)((int)this + 100) = uVar2;
      if (((uVar2 == 0) || (puVar1 <= puVar3)) || (puStack_28 == (undefined1 *)0x0)) {
        if ((uVar2 == 0) && (*(int *)((int)this + 0x54) == 1)) {
          FUN_00616cd0(this_00,unaff_ESI,0);
          *(undefined4 *)((int)this + 100) = *(undefined4 *)((int)this + 0x60);
          if (puVar3 < puStack_2c) {
            FUN_006173b0(this_00,puVar3 + (int)ppiStack_24,(int)puStack_2c - (int)puVar3);
            *(int *)((int)this + 100) = *(int *)((int)this + 100) - this_00[2];
          }
          if (*(int *)((int)this + 0x74) == 0) {
            *(undefined4 *)((int)this + 0x78) = 1;
          }
        }
      }
      else {
        if (unaff_EDI < uVar2) {
          uVar2 = unaff_EDI;
        }
        FUN_006173b0(this_00,puStack_28,uVar2);
        uVar2 = this_00[2];
        iVar4 = *(int *)((int)this + 100) - uVar2;
        *(int *)((int)this + 100) = iVar4;
        if ((uVar2 < unaff_EDI) && (iVar4 == 0)) {
          FUN_00616cd0(this_00,unaff_ESI,0);
          FUN_006173b0(this_00,puStack_28 + uVar2,unaff_EDI - uVar2);
          *(int *)((int)this + 100) = *(int *)((int)this + 0x60) - this_00[2];
        }
      }
      uVar2 = (**(code **)(**(int **)((int)this + 0x34) + 0x4c))
                        (*(int **)((int)this + 0x34),ppiStack_24,puStack_2c,puStack_28);
      if (uVar2 == 0) {
        uVar2 = (uint)(puStack_30 + *(int *)((int)this + 0x58) + unaff_EDI) /
                *(uint *)((int)this + 0x50);
        *(uint *)((int)this + 0x58) =
             (uint)(puStack_30 + *(int *)((int)this + 0x58) + unaff_EDI) %
             *(uint *)((int)this + 0x50);
      }
    }
  }
  return uVar2;
}

