/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package main;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import javax.swing.JOptionPane;

/**
 *
 * @author matthew
 */
public class ServerPrompt extends javax.swing.JPanel {

    /**
     * Creates new form Prompt
     */
    private ServerPrompt() {
        initComponents();
    }

    public static void prompt() {
        ServerPrompt p = new ServerPrompt();

        File confFile = new File("bloodhound/bhserver.conf");
        if (confFile.isFile()) {
            try (BufferedReader read = new BufferedReader(new FileReader(confFile))) {
                p.serverTxt.setText(read.readLine());
                p.portTxt.setText(read.readLine());
                p.usernameTxt.setText(read.readLine());
                p.passwordTxt.setText(read.readLine());
                p.remotePortTxt.setText(read.readLine());
            } catch (IOException ex) {
            }
        }

        if (JOptionPane.showConfirmDialog(null, p, "Configure Bloodhound", JOptionPane.OK_CANCEL_OPTION, JOptionPane.QUESTION_MESSAGE) == JOptionPane.OK_OPTION) {
            try (BufferedWriter writer = new BufferedWriter(new FileWriter(confFile))) {
                writer.write(p.serverTxt.getText() + "\n");
                writer.write(p.portTxt.getText() + "\n");
                writer.write(p.usernameTxt.getText() + "\n");
                writer.write(p.passwordTxt.getText() + "\n");
                writer.write(p.remotePortTxt.getText());
                writer.flush();
            } catch (IOException ex) {
                JOptionPane.showMessageDialog(null, "Failed to save settings.", "Configure Bloodhound", JOptionPane.ERROR_MESSAGE);
            }
        } else {
            JOptionPane.showMessageDialog(null, "Settings not saved.", "Configure Bloodhound", JOptionPane.ERROR_MESSAGE);
        }
    }

    /**
     * This method is called from within the constructor to initialize the form.
     * WARNING: Do NOT modify this code. The content of this method is always
     * regenerated by the Form Editor.
     */
    @SuppressWarnings("unchecked")
    // <editor-fold defaultstate="collapsed" desc="Generated Code">//GEN-BEGIN:initComponents
    private void initComponents() {
        java.awt.GridBagConstraints gridBagConstraints;

        serverTxt = new javax.swing.JTextField();
        serverLbl = new javax.swing.JLabel();
        usernameTxt = new javax.swing.JTextField();
        usernameLbl = new javax.swing.JLabel();
        passwordTxt = new javax.swing.JTextField();
        passwordLbl = new javax.swing.JLabel();
        portTxt = new javax.swing.JTextField();
        portLbl = new javax.swing.JLabel();
        remotePortLbl = new javax.swing.JLabel();
        remotePortTxt = new javax.swing.JTextField();
        jSeparator1 = new javax.swing.JSeparator();

        setLayout(new java.awt.GridBagLayout());

        serverTxt.setText("localhost");
        serverTxt.setPreferredSize(new java.awt.Dimension(200, 20));
        serverTxt.addActionListener(new java.awt.event.ActionListener() {
            public void actionPerformed(java.awt.event.ActionEvent evt) {
                serverTxtActionPerformed(evt);
            }
        });
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 1;
        gridBagConstraints.gridy = 0;
        gridBagConstraints.fill = java.awt.GridBagConstraints.BOTH;
        gridBagConstraints.weightx = 1.0;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(serverTxt, gridBagConstraints);

        serverLbl.setText("Neo4j Server");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 0;
        gridBagConstraints.gridy = 0;
        gridBagConstraints.anchor = java.awt.GridBagConstraints.EAST;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(serverLbl, gridBagConstraints);

        usernameTxt.setText("neo4j");
        usernameTxt.setPreferredSize(new java.awt.Dimension(200, 20));
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 1;
        gridBagConstraints.gridy = 2;
        gridBagConstraints.fill = java.awt.GridBagConstraints.BOTH;
        gridBagConstraints.weightx = 1.0;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(usernameTxt, gridBagConstraints);

        usernameLbl.setText("Username");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 0;
        gridBagConstraints.gridy = 2;
        gridBagConstraints.anchor = java.awt.GridBagConstraints.EAST;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(usernameLbl, gridBagConstraints);

        passwordTxt.setText("neo4j");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 1;
        gridBagConstraints.gridy = 3;
        gridBagConstraints.fill = java.awt.GridBagConstraints.BOTH;
        gridBagConstraints.weightx = 1.0;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(passwordTxt, gridBagConstraints);

        passwordLbl.setText("Password");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 0;
        gridBagConstraints.gridy = 3;
        gridBagConstraints.anchor = java.awt.GridBagConstraints.EAST;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(passwordLbl, gridBagConstraints);

        portTxt.setText("7474");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 1;
        gridBagConstraints.gridy = 1;
        gridBagConstraints.fill = java.awt.GridBagConstraints.BOTH;
        gridBagConstraints.weightx = 1.0;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(portTxt, gridBagConstraints);

        portLbl.setText("Port");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 0;
        gridBagConstraints.gridy = 1;
        gridBagConstraints.anchor = java.awt.GridBagConstraints.EAST;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(portLbl, gridBagConstraints);

        remotePortLbl.setText("Remote Port");
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 0;
        gridBagConstraints.gridy = 5;
        gridBagConstraints.anchor = java.awt.GridBagConstraints.EAST;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(remotePortLbl, gridBagConstraints);

        remotePortTxt.setText("7474");
        remotePortTxt.addActionListener(new java.awt.event.ActionListener() {
            public void actionPerformed(java.awt.event.ActionEvent evt) {
                remotePortTxtActionPerformed(evt);
            }
        });
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 1;
        gridBagConstraints.gridy = 5;
        gridBagConstraints.fill = java.awt.GridBagConstraints.BOTH;
        gridBagConstraints.weightx = 1.0;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(remotePortTxt, gridBagConstraints);
        gridBagConstraints = new java.awt.GridBagConstraints();
        gridBagConstraints.gridx = 0;
        gridBagConstraints.gridy = 4;
        gridBagConstraints.gridwidth = 2;
        gridBagConstraints.fill = java.awt.GridBagConstraints.HORIZONTAL;
        gridBagConstraints.insets = new java.awt.Insets(5, 5, 5, 5);
        add(jSeparator1, gridBagConstraints);
    }// </editor-fold>//GEN-END:initComponents

    private void serverTxtActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_serverTxtActionPerformed
        // TODO add your handling code here:
    }//GEN-LAST:event_serverTxtActionPerformed

    private void remotePortTxtActionPerformed(java.awt.event.ActionEvent evt) {//GEN-FIRST:event_remotePortTxtActionPerformed
        // TODO add your handling code here:
    }//GEN-LAST:event_remotePortTxtActionPerformed


    // Variables declaration - do not modify//GEN-BEGIN:variables
    private javax.swing.JSeparator jSeparator1;
    private javax.swing.JLabel passwordLbl;
    private javax.swing.JTextField passwordTxt;
    private javax.swing.JLabel portLbl;
    private javax.swing.JTextField portTxt;
    private javax.swing.JLabel remotePortLbl;
    private javax.swing.JTextField remotePortTxt;
    private javax.swing.JLabel serverLbl;
    private javax.swing.JTextField serverTxt;
    private javax.swing.JLabel usernameLbl;
    private javax.swing.JTextField usernameTxt;
    // End of variables declaration//GEN-END:variables

}